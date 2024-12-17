
export function debounce<F extends (...args: any[]) => any>(func: F, wait: number) {
    let timeout: NodeJS.Timeout | null = null;
    
    return (...args: Parameters<F>) => {
      if (timeout) {
        clearTimeout(timeout);
      }
      
      timeout = setTimeout(() => {
        func(...args);
      }, wait);
    };
  }