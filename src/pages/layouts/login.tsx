import { Button, Form, Input } from 'antd';
import { useTranslation } from 'react-i18next';

export default function Login() {
  const { t } = useTranslation();
  return (
    <div className="p-10">
      <h1>{t('login_title')}</h1>
      <Form>
        <Form.Item label={t('account')}>
          <Input />
        </Form.Item>
        <Form.Item label={t('password')}>
          <Input type="password" />
        </Form.Item>

        <Button type="primary" htmlType="submit">
          Login
        </Button>
      </Form>
    </div>
  );
}
