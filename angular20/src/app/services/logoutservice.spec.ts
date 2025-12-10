import { TestBed } from '@angular/core/testing';

import { Logoutservice } from './logoutservice';

describe('Logoutservice', () => {
  let service: Logoutservice;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(Logoutservice);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
