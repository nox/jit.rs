initSidebarItems({"enum":[["Abi","A platform's application binary interface"],["ReadElfErrorCode","An error from trying to open the ELF"]],"fn":[["get","Get the Rust type given as a type descriptor"],["init","Initialise the library and prepare for operations"],["supports_threads","Check if the JIT supports theads"],["supports_virtual_memory","Check if the JIT supports virtual memory"],["uses_interpreter","Check if the JIT is using a fallback interpreter"]],"mod":[["flags","Call flags to a function"],["kind","The integer representation of a type"],["typecs","Type constants"]],"struct":[["Block","Represents a single LibJIT block"],["CallFlags",""],["CompiledFunction","A function which has already been compiled from an `UncompiledFunction`, so it can be called but not added to."],["Context","Holds all of the functions you have built and compiled. There can be multiple, but normally there is only one."],["Field","A single field of a struct"],["Fields","Iterates through all the fields of a struct"],["Func","A function"],["Instruction","Represents a single LibJIT instruction"],["InstructionIter",""],["Label","A label in the code that can be branched to in instructions"],["Needed","An ELF dependency iterator"],["Params","Iterator through all the arguments a function takes"],["ReadElf","An ELF binary reader"],["ReadElfError","An error from trying to open the ELF, including the filename"],["TaggedType",""],["Ty","An object that represents a native system type"],["Type","An owned object that represents a native system type."],["TypeKind",""],["UncompiledFunction","A function which has not been compiled yet, so it can have instructions added to it."],["Val","Vals form the backbone of the storage system in `LibJIT`"],["WriteElf","An ELF binary reader"]],"trait":[["Compile","A type that can be compiled into a LibJIT representation"]],"type":[["CowType","A copy-on-write type"],["StaticType","A static type"]]});