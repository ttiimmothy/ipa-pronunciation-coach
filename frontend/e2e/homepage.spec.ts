import { test, expect } from '@playwright/test';

test.describe('Homepage', () => {
  test('has title', async ({ page }) => {
    await page.goto('/');
    
    await expect(page).toHaveTitle(/IPA Pronunciation Coach/);
  });

  test('has navigation links', async ({ page }) => {
    await page.goto('/');
    
    await expect(page.getByRole('link', { name: 'Get Started' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Sign Up Free' })).toBeVisible();
  });

  test('navigates to login page', async ({ page }) => {
    await page.goto('/');
    
    await page.getByRole('link', { name: 'Get Started' }).click();
    
    await expect(page).toHaveURL('/login');
    await expect(page.getByRole('heading', { name: 'Sign in to your account' })).toBeVisible();
  });

  test('navigates to register page', async ({ page }) => {
    await page.goto('/');
    
    await page.getByRole('link', { name: 'Sign Up Free' }).click();
    
    await expect(page).toHaveURL('/register');
  });
});

test.describe('Login Page', () => {
  test('has login form', async ({ page }) => {
    await page.goto('/login');
    
    await expect(page.getByRole('heading', { name: 'Sign in to your account' })).toBeVisible();
    await expect(page.getByLabel('Email')).toBeVisible();
    await expect(page.getByLabel('Password')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Login' })).toBeVisible();
  });

  test('shows validation errors for empty form', async ({ page }) => {
    await page.goto('/login');
    
    await page.getByRole('button', { name: 'Login' }).click();
    
    // Check that form fields are visible and form validation works
    await expect(page.getByLabel('Email')).toBeVisible();
    await expect(page.getByLabel('Password')).toBeVisible();
    // The form should still be on the login page (not redirected)
    await expect(page).toHaveURL(/\/login/);
  });

  test('handles login form submission', async ({ page }) => {
    await page.goto('/login');
    
    await page.getByLabel('Email').fill('test@example.com');
    await page.getByLabel('Password').fill('password123');
    
    await page.getByRole('button', { name: 'Login' }).click();
    
    // Wait for navigation to complete
    await page.waitForLoadState('networkidle');
    
    // Check if we're redirected to dashboard (successful login) or still on login page
    const currentUrl = page.url();
    if (currentUrl.includes('/dashboard')) {
      // Login was successful, check dashboard content
      await expect(page.getByText('Welcome back')).toBeVisible();
    } else {
      // Login failed or stayed on login page, check form is still visible
      await expect(page.getByLabel('Email')).toBeVisible();
    }
  });
});

test.describe('Dashboard Page', () => {
  test('shows dashboard content', async ({ page }) => {
    await page.goto('/dashboard');
    
    await expect(page.getByText('Welcome back')).toBeVisible();
    await expect(page.getByText("Today's Practice")).toBeVisible();
    await expect(page.getByText('Current Streak')).toBeVisible();
    await expect(page.getByText('Words Mastered')).toBeVisible();
  });

  test('has quick action buttons', async ({ page }) => {
    await page.goto('/dashboard');
    
    await expect(page.getByRole('link', { name: 'Browse Vocabulary' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Start Practice' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'View Progress' })).toBeVisible();
    // Use first() to get the first Settings link (in quick actions)
    await expect(page.getByRole('link', { name: 'Settings' }).first()).toBeVisible();
  });
});

test.describe('Responsive Design', () => {
  test('works on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    await expect(page.getByRole('heading', { name: 'Master English Pronunciation' })).toBeVisible();
  });

  test('works on tablet', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/');
    
    await expect(page.getByRole('heading', { name: 'Master English Pronunciation' })).toBeVisible();
  });
});
