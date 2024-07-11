import React from 'react';
import { BrowserRouter, useRoutes } from 'react-router-dom';
import { routes } from './routes.tsx';

const RoutersList = () => {
  const RoutersList = useRoutes(routes);
  return RoutersList;
};
const RouterConfig: React.FC = () => {
  return (
    <BrowserRouter>
      <RoutersList />
    </BrowserRouter>
  );
};
export default RouterConfig;
