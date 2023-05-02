timebomb
========

Retrieval of the Windows timebomb date in multiple languages because for some
reason I thought that was a productive use of my time.

In all of the included samples, the magic number `0x7ffe02c8` comes up. This is
the `&(USER_SHARED_DATA->SystemExpirationDate)` address. `USER_SHARED_DATA` is
the user mode pointer to the [`KUSER_SHARED_DATA`][1] structure, which allows
user mode code to easily access certain system global data without the need to
invoke a system call.

The `SystemExpirationDate` member is documented as follows:
> If the system is an evaluation unit, the following field contains the date and
> time that the evaluation unit expires. A value of 0 indicates that there is no
> expiration. A non-zero value is the UTC absolute time that the system expires.

[1]: https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-kuser_shared_data
