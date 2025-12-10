import { TestBed } from '@angular/core/testing';

import { Mfaservice } from './mfaservice';

describe('Mfaservice', () => {
  let service: Mfaservice;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(Mfaservice);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
