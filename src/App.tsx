import { invoke } from '@tauri-apps/api';
import { getVersion } from '@tauri-apps/api/app';
import { ConfigProvider, Tag, theme } from 'antd';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import RouterConfig from './router';

function App() {
  const [version, setVersion] = useState<string>('');
  useEffect(() => {
    getVersion().then((res) => {
      setVersion(res);
    });
    setTimeout(() => {
      document.addEventListener('DOMContentLoaded', () => {
        // 这将等待窗口加载，但你可以在您想要的任何触发器上运行此函数
        invoke('plugin:commands|close_splashscreen');
      });
    }, 2000);
  }, []);

  const [algorithm] = useState<string>('light');

  // console.log('algorithm', theme.defaultAlgorithm);

  const { i18n } = useTranslation();

  return (
    <ConfigProvider
      componentSize="large"
      variant="outlined"
      theme={{
        algorithm:
          algorithm === 'light' ? theme.defaultAlgorithm : theme.darkAlgorithm,
      }}
    >
      <div
        className="h-screen w-screen"
        style={{
          backgroundColor: algorithm === 'light' ? '#fff' : '#000',
          transition: 'all 0.2s',
        }}
      >
        <button
          onClick={() => {
            i18n.changeLanguage(i18n.language === 'zh' ? 'en' : 'zh');
          }}
        >
          切换-{i18n.language}
        </button>
        <Tag color="#108ee9">v{version}</Tag>
        <RouterConfig />
      </div>
    </ConfigProvider>
  );
}

export default App;
