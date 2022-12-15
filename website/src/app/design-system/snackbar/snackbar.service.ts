import { Injectable } from '@angular/core';
import {MatSnackBar} from '@angular/material/snack-bar';
import { SnackbarComponent } from './snackbar.component';

@Injectable({
  providedIn: 'root'
})
export class SnackbarService {

  constructor(private _snackBar: MatSnackBar) { }

  openSnackBar(message: string, action: string, duration: number) {
    this._snackBar.openFromComponent(SnackbarComponent, {
      duration: duration, data: {message: message}
    });
  }
}
