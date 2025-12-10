import { ComponentFixture, TestBed } from '@angular/core/testing';

import { Prodcatalog } from './prodcatalog';

describe('Prodcatalog', () => {
  let component: Prodcatalog;
  let fixture: ComponentFixture<Prodcatalog>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [Prodcatalog]
    })
    .compileComponents();

    fixture = TestBed.createComponent(Prodcatalog);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
