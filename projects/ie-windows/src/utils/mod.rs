use winreg::enums::{HKEY_CURRENT_USER};
use winreg::RegKey;

// 在注册表中按照以下路径寻找
//
// HKCU/Software/Microsoft/Windows/CurrentVersion/Policies/Explorer/把DisableThumbnails的值从1改为0
//
// 现在再去文件夹选项里就可以看到“始终显示图标从不显示缩略图”的选项了。
pub fn enable_thumbnails() -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let explorer = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer")?;
    explorer.set_value("DisableThumbnails", &0u32)?;
    Ok(())
}

#[test]
fn test() {
    enable_thumbnails().unwrap();
}