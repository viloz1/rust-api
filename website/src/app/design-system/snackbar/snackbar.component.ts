import { Component, OnInit, Input, Inject } from '@angular/core';
import {MatSnackBar, MAT_SNACK_BAR_DATA} from '@angular/material/snack-bar';

@Component({
  selector: 'ds-snackbar',
  templateUrl: './snackbar.component.html',
  styleUrls: ['./snackbar.component.scss']
})
export class SnackbarComponent implements OnInit {

  constructor(@Inject(MAT_SNACK_BAR_DATA) public data: any) { }

  ngOnInit(): void {
  }

}
