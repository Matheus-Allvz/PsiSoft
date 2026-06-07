const { test, expect } = require('@playwright/test');

test.describe('Autenticação API', () => {
  const login = `testuser_${Date.now()}`;
  const password = 'Password123!';

  test('deve registrar um novo psicólogo com sucesso', async ({ request }) => {
    const response = await request.post('/auth/register', {
      data: {
        nome: 'Psicólogo de Teste',
        login: login,
        email: `${login}@example.com`,
        senha: password,
        crp: '06/123456',
        perfil: 'Psicologo'
      }
    });

    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.message).toContain('sucesso');
  });

  test('deve realizar login e receber um token JWT', async ({ request }) => {
    const response = await request.post('/auth/login', {
      data: {
        login: login,
        senha: password
      }
    });

    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.token).toBeDefined();
    expect(typeof body.token).toBe('string');
  });

  test('deve falhar no login com senha incorreta', async ({ request }) => {
    const response = await request.post('/auth/login', {
      data: {
        login: login,
        senha: 'wrong_password'
      }
    });

    expect(response.status()).toBe(401);
  });
});
