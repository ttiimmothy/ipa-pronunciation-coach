import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import LoginForm from '../../components/LoginForm.vue';

// Mock the useAuth composable
vi.mock('../../composables/useAuth', () => ({
  useAuth: vi.fn(() => ({
    login: vi.fn(),
    isLoggingIn: false,
    loginError: null,
  })),
}));

describe('LoginForm', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    const wrapper = mount(LoginForm);
    
    expect(wrapper.find('form').exists()).toBe(true);
    expect(wrapper.find('input[type="email"]').exists()).toBe(true);
    expect(wrapper.find('input[type="password"]').exists()).toBe(true);
    expect(wrapper.find('button[type="submit"]').exists()).toBe(true);
  });

  it('shows validation errors', async () => {
    const wrapper = mount(LoginForm);
    
    // Try to submit without filling fields
    await wrapper.find('form').trigger('submit');
    
    // Wait for validation to run
    await wrapper.vm.$nextTick();
    
    // Should show validation errors in the errors object
    expect(wrapper.vm.errors.email).toBeDefined();
    expect(wrapper.vm.errors.password).toBeDefined();
  });

  it('handles form submission', async () => {
    // Mock the useAuth composable to return loading state
    const { useAuth } = await import('../../composables/useAuth');
    vi.mocked(useAuth).mockReturnValue({
      login: vi.fn(),
      isLoggingIn: true,
      loginError: null,
    } as any);
    
    const wrapper = mount(LoginForm);
    
    // Fill in form
    await wrapper.find('input[type="email"]').setValue('test@example.com');
    await wrapper.find('input[type="password"]').setValue('password123');
    
    // Submit form
    await wrapper.find('form').trigger('submit');
    
    // Wait for async operation
    await wrapper.vm.$nextTick();
    
    // Should show loading state
    expect(wrapper.find('button').text()).toBe('Logging in...');
  });

  it('shows error message on login failure', async () => {
    // Mock the useAuth composable to return an error
    const { useAuth } = await import('../../composables/useAuth');
    vi.mocked(useAuth).mockReturnValue({
      login: vi.fn(),
      isLoggingIn: false,
      loginError: { message: 'Invalid credentials' },
    } as any);
    
    const wrapper = mount(LoginForm);
    
    // Fill in form with invalid credentials
    await wrapper.find('input[type="email"]').setValue('invalid@example.com');
    await wrapper.find('input[type="password"]').setValue('wrongpassword');
    
    // Submit form
    await wrapper.find('form').trigger('submit');
    
    // Wait for async operation
    await wrapper.vm.$nextTick();
    
    // Should show error message
    expect(wrapper.find('.text-red-600').exists()).toBe(true);
  });
});
