# PRX section structure

## .rodata.sceModuleInfo

* Most important section
* Details module information, and points to other sections

## .lib.ent

* Details module exports
  * `module_start`
  * `SceModuleInfo`
  * `module_stop`
  * etc...

## .rodata.sceResident

* Split into 2 uses:
  * The actual export table referenced in .lib.ent. The number of entries
    here is specified in the .lib.ent variable and function count fields.
  * Storing the names of imported resident libraries

## .lib.stub

* Details modules to be imported
* Unknown ATM if this is system imports only or can be additional user modules.

## .sceStub.text

* Contains jump code for imported modules
* 2 instructions per function, usually `jr $ra` followed by `nop`.

## .rodata.sceNid

* Contains lists of NIDs, to be referenced in .lib.stub
