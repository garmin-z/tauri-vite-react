import { listen } from '@tauri-apps/api/event';
import { relaunch } from '@tauri-apps/api/process';
import { invoke } from '@tauri-apps/api/tauri';
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';
import { useClickAway } from 'ahooks';
import { Button, Modal, notification } from 'antd';
import { useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';

type AllowedIps = {
  ip: string;
  response: string;
};

function Connect() {
  async function greet() {
    setShowPanel(true);

    setIps([]);
    invoke('plugin:commands|start_udp_broadcast', {}).then((r) => {
      console.log('response ===>', r);
    });
  }

  const [ips, setIps] = useState<AllowedIps[]>([]);

  useEffect(() => {
    listen('udp_response', (event) => {
      setIps((prev) => [...prev, event.payload as AllowedIps]);
      console.log('event==>', event);
    });
  }, []);

  const [showPanel, setShowPanel] = useState(false);
  const panelRef = useRef<HTMLDivElement>(null);
  useClickAway(() => {
    setShowPanel(false);
    stopUdpListener();
  }, panelRef);

  const [host, setHost] = useState('');

  function onChooseIp(ip: string) {
    setHost(ip);
    setShowPanel(false);
    stopUdpListener();
  }

  function stopUdpListener() {
    invoke('plugin:commands|stop_udp_listener', {}).then((r) => {
      console.log('response ===>', r);
    });
  }

  function genFullApiUrl() {
    let tmp = host.split(':');
    if (tmp.length > 1) {
      if (!tmp[1]) {
        tmp[1] = '18333';
      }
      let url = `http://${tmp[0]}:${tmp[1]}?channel=prod&lang=cn`;
      // console.log(`url: ${url}`)
      return { ip: tmp[0], port: tmp[1], url };
    } else {
      let url = `http://${host}:${18333}?channel=prod&lang=cn`;
      return { ip: host, port: '18333', url };
    }
  }

  const navigate = useNavigate();
  function onToConnectHost() {
    const { url, ip, port } = genFullApiUrl();

    invoke('plugin:commands|check_host_allow', { data: { ip, port } }).then(
      (r) => {
        console.log('response ===>', r);
        navigate(`/studio?src=${url}`);
      },
    );
    // if (this.selected_port === 18333 || this.selected_port === '18333') {
    //   this.connectTip = this.$t('message.connectingTip')
    //   axios.get(`http://${this.selected_ip}:${this.selected_port}/check`, {
    //     timeout: 5000
    //   })
    //     .then((res) => {
    //       console.log(`check success, code=${res.status}`);
    //       this.connectTip = '';
    //       this.$router.push({ name: 'UFACTORY-STUDIO-FRAME', params: { src: addr } });
    //       // window.location = addr;
    //     })
    //     .catch((e) => {
    //       console.log(`check failed, ${e}`);
    //       this.connectTip = this.$t('message.connectFailTip')
    //       setTimeout(() => {
    //         this.connectTip = '';
    //       }, 3000)
    //     })
    // }
    // else {
    //   this.$router.push({ name: 'UFACTORY-STUDIO-FRAME', params: { src: addr } });
    //   // window.location = addr;
    // }
  }

  function onCheckUpdate() {
    console.log('onCheckUpdate');

    checkUpdate().then((res) => {
      console.log('res', res);

      const { shouldUpdate, manifest } = res;
      console.log('shouldUpdate');

      if (shouldUpdate) {
        Modal.confirm({
          title: `发现新版本：${manifest?.version}`,
          content: `是否升级？`,
          okText: '升级',
          cancelText: '取消',
          onOk: async () => {
            try {
              notification.info({
                message: '正在下载更新...',
                duration: 3000,
              });
              await installUpdate();
              await relaunch();
            } catch (e) {
              notification.error({
                message: '下载更新失败',
                description: e?.toString() || '',
              });
            }
          },
        });
      }
    });
  }

  function onopenSetting() {
    invoke('plugin:commands|open_setting_page').then((r) => {
      console.log('response ===>', r);
    });
  }
  return (
    <div className="h-screen w-screen bg-gray-100">
      <Button onClick={onCheckUpdate}>update</Button>
      <Button onClick={onopenSetting}>Setting Page</Button>
      <div className="absolute left-1/2 top-1/2 flex w-screen -translate-x-1/2 -translate-y-2/3 flex-col justify-center gap-y-10">
        <div className="text-center font-mono text-5xl font-light">
          UFACTORY Studio
        </div>
        <form
          className="flex justify-center text-center"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <div className="relative w-1/2 max-w-2xl">
            <input
              className="h-12 w-full border-0 bg-white text-center outline-0"
              id="greet-input"
              onChange={(e) => setHost(e.currentTarget.value)}
              value={host}
              placeholder="Enter a name..."
            />
            {showPanel && (
              <div
                className="absolute left-0 h-64 w-full overflow-auto bg-gray-200"
                ref={panelRef}
              >
                {ips.map((item, i) => (
                  <div
                    className="line-clamp-1 h-10 w-full cursor-pointer text-center text-xs leading-10 text-gray-600 hover:bg-gray-300"
                    key={i}
                    onClick={() => onChooseIp(item.ip)}
                  >
                    {i + 1}.{item.ip}, {item.response}
                  </div>
                ))}
              </div>
            )}
          </div>

          <button
            type="submit"
            className="h-12 w-28 bg-gray-400 text-white transition-all hover:bg-gray-500"
          >
            搜索服务器
          </button>
        </form>
        <button
          className={
            'm-auto mt-10 h-12 w-28 rounded-sm text-white transition-all ' +
            (host ? 'bg-blue-500' : 'bg-gray-400')
          }
          onClick={() => onToConnectHost()}
        >
          连接
        </button>
      </div>
    </div>
  );
}

export default Connect;
