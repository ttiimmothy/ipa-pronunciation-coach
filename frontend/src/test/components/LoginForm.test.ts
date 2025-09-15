import { describe, it, expect, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import LoginForm from '../../components/LoginForm.vue';

describe('LoginForm', () => {
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
    
    // Should show required field validation
    expect(wrapper.find('input[type="email"]').attributes('required')).toBeDefined();
    expect(wrapper.find('input[type="password"]').attributes('required')).toBeDefined();
  });

  it('handles form submission', async () => {
    const wrapper = mount(LoginForm);
    
    // Fill in form
    await wrapper.find('input[type="email"]').setValue('test@example.com');
    await wrapper.find('input[type="password"]').setValue('password123');
    
    // Mock window.location
    const mockLocation = { href: '' };
    Object.defineProperty(window, 'location', {
      value: mockLocation,
      writable: true,
    });
    
    // Submit form
    await wrapper.find('form').trigger('submit');
    
    // Should show loading state
    expect(wrapper.find('button').text()).toBe('Logging in...');
  });

  it('shows error message on login failure', async () => {
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
