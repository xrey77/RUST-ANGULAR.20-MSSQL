import { ComponentFixture, TestBed } from '@angular/core/testing';

import { Prodsearch } from './prodsearch';

describe('Prodsearch', () => {
  let component: Prodsearch;
  let fixture: ComponentFixture<Prodsearch>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [Prodsearch]
    })
    .compileComponents();

    fixture = TestBed.createComponent(Prodsearch);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
