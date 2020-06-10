# Example

```rust
use lpc55_bootrom_sys::{flash_config_t, FLASH_Init};
let config = unsafe {
    let mut config = core::mem::MaybeUninit::<flash_config_t>::zeroed();
    FLASH_Init(config.as_mut_ptr());
    config.assume_init()
};
panic!("{:?}", &config);
```
