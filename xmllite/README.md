# xmllite

This is the spoofed xmllite library.

It is designed to override the original library by reexporting the original functions.
It also creates a new thread on the load which runs the desired functionality.

### WARNING
Do not replace the original xmllite library in the System32 directory as it will cause the system to be unable to boot up.

Good idea is to put the compiled binary into the folder of an application that depends on xmllite.dll