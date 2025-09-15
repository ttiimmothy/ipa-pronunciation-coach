import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import RecorderButton from '../../components/RecorderButton.vue';

describe('RecorderButton', () => {
  beforeEach(() => {
    // Reset mocks
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    const wrapper = mount(RecorderButton);
    
    expect(wrapper.find('button').exists()).toBe(true);
    expect(wrapper.find('svg').exists()).toBe(true);
    expect(wrapper.text()).toContain('Tap to Record');
  });

  it('shows recording state when clicked', async () => {
    const wrapper = mount(RecorderButton);
    
    // Mock MediaRecorder
    const mockMediaRecorder = {
      start: vi.fn(),
      stop: vi.fn(),
      ondataavailable: null,
      onstop: null,
    };
    
    (global.MediaRecorder as any).mockImplementation(() => mockMediaRecorder);
    
    // Mock getUserMedia
    const mockStream = new MediaStream();
    vi.mocked(navigator.mediaDevices.getUserMedia).mockResolvedValue(mockStream);
    
    // Click record button
    await wrapper.find('button').trigger('click');
    
    // Should show recording state
    expect(wrapper.text()).toContain('Recording...');
    expect(wrapper.find('button').classes()).toContain('bg-red-500');
  });

  it('emits start event when recording starts', async () => {
    const wrapper = mount(RecorderButton);
    
    // Mock MediaRecorder
    const mockMediaRecorder = {
      start: vi.fn(),
      stop: vi.fn(),
      ondataavailable: null,
      onstop: null,
    };
    
    (global.MediaRecorder as any).mockImplementation(() => mockMediaRecorder);
    
    // Mock getUserMedia
    const mockStream = new MediaStream();
    vi.mocked(navigator.mediaDevices.getUserMedia).mockResolvedValue(mockStream);
    
    // Click record button
    await wrapper.find('button').trigger('click');
    
    // Should emit start event
    expect(wrapper.emitted('start')).toBeTruthy();
  });

  it('handles recording errors gracefully', async () => {
    const wrapper = mount(RecorderButton);
    
    // Mock getUserMedia to reject
    vi.mocked(navigator.mediaDevices.getUserMedia).mockRejectedValue(new Error('Permission denied'));
    
    // Click record button
    await wrapper.find('button').trigger('click');
    
    // Should emit error event
    expect(wrapper.emitted('error')).toBeTruthy();
  });

  it('shows disabled state when disabled prop is true', () => {
    const wrapper = mount(RecorderButton, {
      props: {
        disabled: true,
      },
    });
    
    expect(wrapper.find('button').attributes('disabled')).toBeDefined();
    expect(wrapper.find('button').classes()).toContain('cursor-not-allowed');
  });

  it('respects max duration limit', async () => {
    const wrapper = mount(RecorderButton, {
      props: {
        maxDuration: 2, // 2 seconds
      },
    });
    
    // Mock MediaRecorder
    const mockMediaRecorder = {
      start: vi.fn(),
      stop: vi.fn(),
      ondataavailable: null,
      onstop: null,
    };
    
    (global.MediaRecorder as any).mockImplementation(() => mockMediaRecorder);
    
    // Mock getUserMedia
    const mockStream = new MediaStream();
    vi.mocked(navigator.mediaDevices.getUserMedia).mockResolvedValue(mockStream);
    
    // Start recording
    await wrapper.find('button').trigger('click');
    
    // Fast-forward time
    vi.advanceTimersByTime(3000); // 3 seconds
    
    // Should have stopped recording due to max duration
    expect(mockMediaRecorder.stop).toHaveBeenCalled();
  });
});
