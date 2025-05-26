const express = require("express");
const { createClient } = require("@supabase/supabase-js");
const { BorshCoder } = require("@coral-xyz/anchor");
const idl = require("./idl.json"); // Load your program's IDL

const app = express();
app.use(express.json({ limit: "10mb" })); // Increase limit for large payloads

// Initialize Supabase client
const supabase = createClient(
  "https://gfctlllkdwmdjbneddwy.supabase.co",
  "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImdmY3RsbGxrZHdtZGpibmVkZHd5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDc0MTU5NjksImV4cCI6MjA2Mjk5MTk2OX0.cvnqXMozlnSpzNkQAkAq82aitaNW6hmznmtUMZJ7GSw"
);

// Initialize Anchor coder for event decoding
const coder = new BorshCoder(idl);

// Your program ID
const PROGRAM_ID = "QaQX5WUroY6mHE8RPXXiQUnU73YFRVwKGkSaFcFj6yw";

// Helper function to decode events from logs
function decodeEventsFromLogs(logs, programId) {
  const events = [];

  for (const log of logs) {
    try {
      // Look for program data logs (these contain events)
      if (log.startsWith("Program data: ")) {
        const base64Data = log.substring("Program data: ".length).trim();
        const binaryData = Buffer.from(base64Data, "base64");

        // Try to decode as an event
        const decodedEvent = coder.events.decode(binaryData);
        if (decodedEvent) {
          events.push(decodedEvent);
        }
      }
      // Also check for "Program log: " entries that might contain events
      else if (log.startsWith("Program log: ")) {
        const logData = log.substring("Program log: ".length).trim();

        // Check if it looks like base64 encoded data
        if (logData.match(/^[A-Za-z0-9+/]+=*$/)) {
          try {
            const binaryData = Buffer.from(logData, "base64");
            const decodedEvent = coder.events.decode(binaryData);
            if (decodedEvent) {
              events.push(decodedEvent);
            }
          } catch (e) {
            // Not all program logs are events, so this is expected
          }
        }
      }
    } catch (error) {
      // Continue processing other logs if one fails
      console.warn("Failed to decode log:", log, "Error:", error.message);
    }
  }

  return events;
}

// Health check endpoint
app.get("/health", (req, res) => {
  res.json({ status: "ok", timestamp: new Date().toISOString() });
});

// Webhook endpoint
app.post("/webhook", async (req, res) => {
  try {
    console.log("=== WEBHOOK RECEIVED ===");
    console.log("Raw payload:", JSON.stringify(req.body, null, 2));
    
    const transactions = Array.isArray(req.body) ? req.body : [req.body];
    console.log(`Processing ${transactions.length} transactions`);

    let totalEventsProcessed = 0;

    for (const transaction of transactions) {
      try {
        // Check if this transaction involves our program
        const accountKeys = transaction.transaction?.message?.accountKeys || [];
        console.log("Account keys in transaction:", accountKeys);
        
        const isProgramInvolved = accountKeys.some((key) => key === PROGRAM_ID);
        console.log("Is our program involved?", isProgramInvolved);

        if (!isProgramInvolved) {
          console.log("Skipping transaction - program not involved");
          continue; // Skip transactions that don't involve our program
        }

        const logMessages = transaction.meta?.logMessages || [];
        console.log("Log messages:", logMessages);
        
        const signature = transaction.transaction?.signatures?.[0];
        const blockTime = transaction.blockTime;

        if (!signature) {
          console.warn("Transaction missing signature, skipping");
          continue;
        }

        // Decode events from logs
        const events = decodeEventsFromLogs(logMessages, PROGRAM_ID);
        console.log(`Decoded ${events.length} events:`, events);

        // Store events in Supabase
        for (const event of events) {
          try {
            const eventData = {
              event_type: event.name,
              transaction_hash: signature,
              block_time: blockTime ? new Date(blockTime * 1000) : new Date(),
              data: event.data,
              program_id: PROGRAM_ID,
            };

            console.log("Inserting event data:", eventData);

            const { data, error } = await supabase
              .from("events")
              .insert(eventData);

            if (error) {
              console.error(
                "Error inserting event:",
                error.message,
                "Event:",
                event.name
              );
            } else {
              console.log(
                "Event inserted successfully:",
                event.name,
                "Signature:",
                signature
              );
              totalEventsProcessed++;
            }
          } catch (insertError) {
            console.error("Database insertion error:", insertError.message);
          }
        }

        if (events.length > 0) {
          console.log(
            `Processed ${events.length} events from transaction ${signature}`
          );
        }
      } catch (transactionError) {
        console.error(
          "Error processing transaction:",
          transactionError.message
        );
      }
    }

    console.log(
      `Webhook processing complete. Total events processed: ${totalEventsProcessed}`
    );
    res.status(200).json({
      success: true,
      eventsProcessed: totalEventsProcessed,
      timestamp: new Date().toISOString(),
    });
  } catch (error) {
    console.error("Webhook error:", error.message);
    res.status(500).json({
      success: false,
      error: error.message,
      timestamp: new Date().toISOString(),
    });
  }
});

// Start server
const port = process.env.PORT || 3000;
app.listen(port, () => {
  console.log(`NFT Marketplace webhook server running on port ${port}`);
  console.log(`Program ID: ${PROGRAM_ID}`);
  console.log(`Health check: http://localhost:${port}/health`);
  console.log(`Webhook endpoint: http://localhost:${port}/webhook`);
});
