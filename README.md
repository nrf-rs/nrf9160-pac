nrf9160-pac
===========

This crate is a Peripheral Access Crate (or PAC) for the nRF9160 generated using [svd2rust]
and the official [SVD file][svd]. A Peripheral Access Crate contains low-level register defintions.

For most purposes, you want the [HAL] instead.

Regenerating
------------

To regenerate the crate, first make sure that [svd2rust] and [form] are
installed and up-to-date. Put `nrf9160.svd` into the repo directory and run:

```bash
# Fixup
sed -i 's/read-writeonce/read-writeOnce/' nrf9160.svd

# Generate
svd2rust -i nrf9160.svd
form -i lib.rs -o src
cargo fmt

# Clean up
rm lib.rs
```

Note that some things in the generated Rust code might need to be fixed manually
after these steps.

[HAL]: https://github.com/nrf-rs/nrf52-hal/tree/master/nrf9160-hal
[svd2rust]: https://github.com/japaric/svd2rust
[svd]: https://github.com/NordicSemiconductor/nrfx/blob/master/mdk/nrf9160.svd
[form]: https://github.com/djmcgill/form
