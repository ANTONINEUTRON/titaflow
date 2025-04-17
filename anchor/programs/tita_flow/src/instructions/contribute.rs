use anchor_lang::prelude::*;

// takes flow account, 
// first check rules to determine if contribution rules are met
// if flow type is send only creator and senders can contribute
// if flow type is receive, anyone can contribute