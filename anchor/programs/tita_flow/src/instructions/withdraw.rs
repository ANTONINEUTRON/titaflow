use anchor_lang::prelude::*;

// withdraw from a flow
// if flow type is send, debit the flow account and credit the receiver if in receiver array passed to the function
// if flow type is receive, debit the flow and credit the creator or receiver array of pubkey if specified