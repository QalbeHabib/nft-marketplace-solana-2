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
const PROGRAM_ID = "Equiqs1Z5Q4F1gBuciqo6yrvqNERzwp5v9Fskhq2A5WB";

// Helper function to decode events from logs
function decodeEventsFromLogs(logs, programId) {
  const events = [];

  console.log(
    `Attempting to decode events from ${logs.length} logs for program ${programId}`
  );

  // Look for NFT minting patterns in logs
  let nftMintEvent = null;
  let collectionMintEvent = null;
  let mintAddress = null;
  let ownerAddress = null;

  // First pass to gather context
  for (const log of logs) {
    // Look for mint address
    if (
      log.includes("Associated Token Account Program") &&
      log.includes("Create")
    ) {
      // The next few logs might contain token account info
      console.log("Found token account creation");
    }

    // Look for NFT minting indicators
    if (log.includes("Program log: Minting and verifying NFT")) {
      console.log("Found NFT minting operation");
    }

    // Look for collection creation indicators
    if (
      log.includes("Program log: Creating collection") ||
      log.includes("Create Metadata Accounts") ||
      (log.includes("collection") && log.includes("create"))
    ) {
      console.log("Found potential collection creation operation");
      collectionMintEvent = {
        name: "CollectionMinted",
        data: {
          success: true,
          timestamp: new Date().toISOString(),
        },
      };
    }

    // Check for successful NFT minting
    if (log.includes("NFT minted and verified in collection successfully")) {
      console.log("Found successful NFT minting confirmation");
      nftMintEvent = {
        name: "NftMinted",
        data: {
          success: true,
          timestamp: new Date().toISOString(),
        },
      };
    }

    // Check for successful collection creation
    if (
      log.includes("Collection created successfully") ||
      (log.includes("success") && log.includes("collection"))
    ) {
      console.log("Found successful collection creation confirmation");
      if (collectionMintEvent) {
        collectionMintEvent.data.confirmation = true;
      }
    }
  }

  // Now process the Program data logs which contain the serialized event data
  for (const log of logs) {
    try {
      // Look for program data logs (these contain events)
      if (log.startsWith("Program data: ")) {
        const base64Data = log.substring("Program data: ".length).trim();
        console.log(`Found Program data: ${base64Data.substring(0, 20)}...`);

        try {
          const binaryData = Buffer.from(base64Data, "base64");
          console.log(`Decoded binary data length: ${binaryData.length} bytes`);

          // Try to decode as an event
          const decodedEvent = coder.events.decode(binaryData);
          console.log(
            `Decoded event result: ${
              decodedEvent ? JSON.stringify(decodedEvent) : "null"
            }`
          );

          if (decodedEvent) {
            events.push(decodedEvent);
          } else if (nftMintEvent && events.length === 0) {
            // If we couldn't decode the event but we detected NFT minting
            // and haven't added any events yet, use our detected event
            events.push(nftMintEvent);
          } else if (collectionMintEvent && events.length === 0) {
            // If we couldn't decode the event but we detected collection creation
            events.push(collectionMintEvent);
          }
        } catch (decodeError) {
          console.error(`Error decoding base64 data: ${decodeError.message}`);

          // If decoding failed but we have a detected event, use that
          if (nftMintEvent && events.length === 0) {
            events.push(nftMintEvent);
          } else if (collectionMintEvent && events.length === 0) {
            events.push(collectionMintEvent);
          }
        }
      }
      // Also check for "Program log: " entries that might contain events
      else if (log.startsWith("Program log: ")) {
        const logData = log.substring("Program log: ".length).trim();

        // Check if it's JSON data
        if (logData.startsWith("{") && logData.endsWith("}")) {
          console.log(`Found potential JSON data in program log: ${logData}`);
          try {
            const jsonData = JSON.parse(logData);
            // Add as a custom event
            events.push({
              name: "JsonLogEvent",
              data: jsonData,
            });
            console.log("Added JSON data as event");
          } catch (jsonError) {
            // Not valid JSON
            console.log("Not valid JSON data");
          }
        }
        // Check if it's a mint instruction
        else if (
          logData.includes("Instruction: Mint") ||
          logData.includes("NFT minted")
        ) {
          console.log(`Found minting instruction: ${logData}`);
          // We'll collect this information for our fallback event
          if (nftMintEvent) {
            nftMintEvent.data.instruction = logData;
          }
        }
        // Check if it's a collection creation instruction
        else if (
          logData.includes("Instruction: Create") ||
          (logData.includes("collection") && logData.includes("creat"))
        ) {
          console.log(`Found collection creation instruction: ${logData}`);
          // We'll collect this information for our fallback event
          if (collectionMintEvent) {
            collectionMintEvent.data.instruction = logData;
          }
        }
        // Check if it looks like base64 encoded data
        else if (logData.match(/^[A-Za-z0-9+/]+=*$/)) {
          console.log(`Found potential base64 data in program log: ${logData}`);
          try {
            const binaryData = Buffer.from(logData, "base64");
            console.log(`Decoded binary length: ${binaryData.length} bytes`);

            const decodedEvent = coder.events.decode(binaryData);
            console.log(
              `Decoded event from program log: ${
                decodedEvent ? JSON.stringify(decodedEvent) : "null"
              }`
            );

            if (decodedEvent) {
              events.push(decodedEvent);
            }
          } catch (e) {
            console.log(`Not a decodable event: ${e.message}`);
          }
        }
      }
    } catch (error) {
      // Continue processing other logs if one fails
      console.warn("Failed to decode log:", log, "Error:", error.message);
    }
  }

  // If we detected an event but couldn't decode any events, add our fallback event
  if ((nftMintEvent || collectionMintEvent) && events.length === 0) {
    console.log("Adding fallback event");
    if (nftMintEvent) {
      events.push(nftMintEvent);
    } else if (collectionMintEvent) {
      events.push(collectionMintEvent);
    }
  }

  // If we still have no events but we know our program is involved,
  // create a generic transaction event
  if (events.length === 0) {
    console.log(
      "No specific events detected, adding generic transaction event"
    );
    events.push({
      name: "ProgramTransaction",
      data: {
        timestamp: new Date().toISOString(),
        program_id: programId,
        description: "Transaction involving program detected",
      },
    });
  }

  console.log(`Total events decoded: ${events.length}`);
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
    console.log(`Received webhook at: ${new Date().toISOString()}`);

    const transactions = Array.isArray(req.body) ? req.body : [req.body];
    console.log(`Processing ${transactions.length} transactions`);

    let totalEventsProcessed = 0;

    for (const transaction of transactions) {
      try {
        const signature = transaction.transaction?.signatures?.[0];
        if (!signature) {
          console.warn("Transaction missing signature, skipping");
          continue;
        }

        console.log(`Processing transaction: ${signature}`);

        // Check if this transaction involves our program
        const accountKeys = transaction.transaction?.message?.accountKeys || [];

        // For debugging - print account keys for better visibility
        if (Array.isArray(accountKeys)) {
          console.log("Account keys as list:");
          accountKeys.forEach((key, index) => {
            console.log(`[${index}] ${key}`);
          });
        }

        // Get log messages
        const logMessages = transaction.meta?.logMessages || [];
        if (!logMessages || logMessages.length === 0) {
          console.log("No log messages found in transaction");
          continue;
        }

        // Check if our program is involved
        const isProgramInvolved = accountKeys.some((key) => key === PROGRAM_ID);
        const programMentionedInLogs = logMessages.some((log) =>
          log.includes(PROGRAM_ID)
        );

        console.log("Is our program involved in accounts?", isProgramInvolved);
        console.log(
          "Is our program mentioned in logs?",
          programMentionedInLogs
        );

        // Allow processing if program is mentioned in logs or account keys
        if (!isProgramInvolved && !programMentionedInLogs) {
          console.log("Skipping transaction - program not involved");
          continue; // Skip transactions that don't involve our program
        }

        // Get blockTime for timestamp
        const blockTime = transaction.blockTime;
        const timestamp = blockTime ? new Date(blockTime * 1000) : new Date();

        // Decode events from logs
        const events = decodeEventsFromLogs(logMessages, PROGRAM_ID);
        console.log(
          `Decoded ${events.length} events from transaction ${signature}`
        );

        // Store events in Supabase
        for (const event of events) {
          try {
            const eventData = {
              event_type: event.name,
              transaction_hash: signature,
              block_time: timestamp,
              data: event.data || {},
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
