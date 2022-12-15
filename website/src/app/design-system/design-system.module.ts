import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { InputComponent } from './input/input.component';
import { MatInputModule } from '@angular/material/input';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ButtonComponent } from './button/button.component';
import {MatButtonModule} from '@angular/material/button';
import { SnackbarComponent } from './snackbar/snackbar.component';
import { MatSnackBarModule } from '@angular/material/snack-bar';
import { MaterialModule } from './material/material.module';
import { MatCardModule } from '@angular/material/card';
import { CardComponent } from './card/card.component';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
@NgModule({
  declarations: [
    InputComponent,
    ButtonComponent,
    SnackbarComponent,
    CardComponent
  ],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    MaterialModule,
    FontAwesomeModule
  ], exports: [
    InputComponent,
    ButtonComponent,
    SnackbarComponent,
    MaterialModule,
    CardComponent,
    FontAwesomeModule,
  ]
})
export class DesignSystemModule { }
