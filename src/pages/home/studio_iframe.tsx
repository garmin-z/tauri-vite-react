import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';

function StudioIframe() {
  const [targetSrc, setSrc] = useState('');

  const location = useLocation();
  const params = new URLSearchParams(location.search);

  useEffect(() => {
    setSrc(params.get('src') || targetSrc);
  }, []);
  return (
    <>
      <div className="h-screen w-screen">
        <iframe
          src={targetSrc}
          frameBorder={0}
          height="100%"
          width="100%"
        ></iframe>
      </div>
    </>
  );
}

export default StudioIframe;
