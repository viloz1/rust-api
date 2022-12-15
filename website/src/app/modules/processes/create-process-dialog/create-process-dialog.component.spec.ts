import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CreateProcessDialogComponent } from './create-process-dialog.component';

describe('CreateProcessDialogComponent', () => {
  let component: CreateProcessDialogComponent;
  let fixture: ComponentFixture<CreateProcessDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ CreateProcessDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CreateProcessDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
