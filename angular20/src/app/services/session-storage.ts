import { Injectable, Inject, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';

@Injectable({
  providedIn: 'root'
})
export class SessionStorage {
  private isBrowser: boolean;

  constructor(@Inject(PLATFORM_ID) private platformId: object) {
    this.isBrowser = isPlatformBrowser(this.platformId);
  }

  // setItem(key: string, value: any): void {
  //   sessionStorage.setItem(key, JSON.stringify(value));
  // }
  setItem(key: string, value: string): void {
    if (this.isBrowser) {
      sessionStorage.setItem(key, value);
    }
  }

  getItem(key: string): string | null {
    if (this.isBrowser) {
      return sessionStorage.getItem(key);
    }
    return null; // Return null if not in a browser environment
  }  
  // getItem<T>(key: string): T | null {
  //   try {
  //     const storedValue = sessionStorage.getItem(key);
  //     if (storedValue !== null) {
  //       // Parse the JSON string back into its original type
  //       return JSON.parse(storedValue) as T;
  //     }
  //   } catch (error) {
  //     console.error('Error retrieving from session storage', error);
  //   }
  //   return null;
  // }
  
  // removeItem(key: string): void {
  //   sessionStorage.removeItem(key);
  // }
  removeItem(key: string): void {
    if (this.isBrowser) {
      sessionStorage.removeItem(key);
    }
  }

  clear(): void {
    if (this.isBrowser) {
      sessionStorage.clear();
    }
  }  
}
