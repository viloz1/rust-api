import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatInputModule } from '@angular/material/input';
import { MatSnackBarModule } from '@angular/material/snack-bar';

let material = [
  CommonModule,
  MatButtonModule,
  MatCardModule,
  MatInputModule,
  MatCardModule,
  MatSnackBarModule
]

@NgModule({
  declarations: [],
  imports: material,
  exports: material
})
export class MaterialModule { }
