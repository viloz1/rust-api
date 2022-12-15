import { TestBed } from '@angular/core/testing';

import { ApiProcessesService } from './api-processes.service';

describe('ApiProcessesService', () => {
  let service: ApiProcessesService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ApiProcessesService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
