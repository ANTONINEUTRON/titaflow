use anchor_lang::prelude::*;

// create a flow account
// if flow type is send, debit creator wallet to fund the flow token account
// if flow type is receive, just specify flow fields