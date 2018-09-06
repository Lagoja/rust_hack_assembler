# Simple Assembler for the Hack Machine Language

##TODOs: 
* Find a way to parse the code without Regular Expressions (build a tokenizer)
    * Potential Tokens: 
        1. Symbol (denoted by @)
        2. Dest (denoted by characters before =)
        3. Comp (denoted by characters after = or before ;)
        4. Jump (denoted by 3 char sequence after ;)
        5. Label (denoted by (Xxx))
        6. Comment (denoted by //Xxx)
* Add better error handling (manage Results instead of using unwraps)
* Improve test suite