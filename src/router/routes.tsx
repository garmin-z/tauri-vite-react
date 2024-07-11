import { lazy } from 'react';
import { Navigate, RouteObject } from 'react-router-dom';

const Home = lazy(() => import('../pages/home'));

const Container = lazy(() => import('../pages/layouts/container'));
const Login = lazy(() => import('../pages/layouts/login'));

const StudioIframe = lazy(() => import('../pages/home/studio_iframe'));
const Setting = lazy(() => import('../pages/home/setting'));

export const routes: RouteObject[] = [
  {
    path: '/login',
    element: <Login />,
  },
  {
    path: '/',
    element: <Navigate to="/connect" replace />,
  },
  {
    path: '/',
    element: <Container />,
    children: [
      {
        path: '/connect',
        element: <Home />,
      },
      {
        path: '/studio',
        element: <StudioIframe />,
      },
      {
        path: '/setting',
        element: <Setting />,
      },
    ],
  },
];
