import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleStrategy } from '@angular/router';
import { ApiProcessesService } from 'src/app/services/api-processes.service';

@Component({
  selector: 'app-create-process-dialog',
  templateUrl: './create-process-dialog.component.html',
  styleUrls: ['./create-process-dialog.component.scss']
})
export class CreateProcessDialogComponent implements OnInit {

  constructor(private api: ApiProcessesService) { }


  name = new FormControl('', Validators.required)
  start = new FormControl('', Validators.required)
  stop = new FormControl('', Validators.required)
  build = new FormControl('', Validators.required)
  git = new FormControl('', Validators.required)
  branch = new FormControl('', Validators.required)


  ngOnInit(): void {
  }

  allValid() {
    return(
      this.name.valid &&
      this.start.valid &&
      this.stop.valid &&
      this.build.valid &&
      this.git.valid &&
      this.branch.valid
    )
  }

  add() {
    console.log("Add");
    if(this.name.value && this.start.value && this.stop.value && this.build.value && this.git.value && this.branch.value) {
      this.api.create(this.name.value, this.start.value, this.stop.value, this.build.value, this.git.value, this.branch.value).subscribe((data) => {
        console.log(data);
      });
    }
  }

}
