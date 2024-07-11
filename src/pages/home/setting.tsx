import { invoke } from '@tauri-apps/api/tauri';
import { useEffect } from 'react';

export default function Setting() {
  useEffect(() => {
    invoke('plugin:commands|show_setting_page');
  }, []);
  return <div>Setting</div>;
}
