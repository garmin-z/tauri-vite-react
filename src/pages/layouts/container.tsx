import { useOutlet } from 'react-router-dom';

function Container() {
  const outlet = useOutlet();

  return <div>{outlet}</div>;
}

export default Container;
