import { test, expect } from '@playwright/test';

test.describe('Homepage', () => {
  test('has title', async ({ page }) => {
    await page.goto('/');
    
    await expect(page).toHaveTitle(/IPA Pronunciation Coach/);
  });

  test('has navigation links', async ({ page }) => {
    await page.goto('/');
    
    await expect(page.getByRole('link', { name: 'Login' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Register' })).toBeVisible();
  });

  test('navigates to login page', async ({ page }) => {
    await page.goto('/');
    
    await page.getByRole('link', { name: 'Login' }).click();
    
    await expect(page).toHaveURL('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('navigates to register page', async ({ page }) => {
    await page.goto('/');
    
    await page.getByRole('link', { name: 'Register' }).click();
    
    await expect(page).toHaveURL('/register');
  });
});

test.describe('Login Page', () => {
  test('has login form', async ({ page }) => {
    await page.goto('/login');
    
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await expect(page.getByLabel('Email')).toBeVisible();
    await expect(page.getByLabel('Password')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Login' })).toBeVisible();
  });

  test('shows validation errors for empty form', async ({ page }) => {
    await page.goto('/login');
    
    await page.getByRole('button', { name: 'Login' }).click();
    
    // HTML5 validation should prevent submission
    await expect(page.getByLabel('Email')).toHaveAttribute('required');
    await expect(page.getByLabel('Password')).toHaveAttribute('required');
  });

  test('handles login form submission', async ({ page }) => {
    await page.goto('/login');
    
    await page.getByLabel('Email').fill('test@example.com');
    await page.getByLabel('Password').fill('password123');
    
    await page.getByRole('button', { name: 'Login' }).click();
    
    // Should show loading state
    await expect(page.getByRole('button', { name: 'Logging in...' })).toBeVisible();
  });
});

test.describe('Dashboard Page', () => {
  test('shows dashboard content', async ({ page }) => {
    await page.goto('/dashboard');
    
    await expect(page.getByText('Welcome back')).toBeVisible();
    await expect(page.getByText("Today's Progress")).toBeVisible();
    await expect(page.getByText('Current Streak')).toBeVisible();
    await expect(page.getByText('Words Learned')).toBeVisible();
  });

  test('has quick action buttons', async ({ page }) => {
    await page.goto('/dashboard');
    
    await expect(page.getByRole('link', { name: 'Browse Vocabulary' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Start Practice' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'View Progress' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Settings' })).toBeVisible();
  });
});

test.describe('Responsive Design', () => {
  test('works on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    await expect(page.getByRole('heading', { name: 'IPA Pronunciation Coach' })).toBeVisible();
  });

  test('works on tablet', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/');
    
    await expect(page.getByRole('heading', { name: 'IPA Pronunciation Coach' })).toBeVisible();
  });
});
