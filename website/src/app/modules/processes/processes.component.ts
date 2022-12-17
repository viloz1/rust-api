import { Component, OnInit } from '@angular/core';
import { Process, ProcessesModel } from 'src/app/models/Processes';
import { ApiProcessesService } from 'src/app/services/api-processes.service';
import { CreateProcessDialogComponent } from './create-process-dialog/create-process-dialog.component';
import {MatDialog} from '@angular/material/dialog';
import { faPlus } from '@fortawesome/free-solid-svg-icons';


@Component({
  selector: 'app-processes',
  templateUrl: './processes.component.html',
  styleUrls: ['./processes.component.scss']
})
export class ProcessesComponent implements OnInit {

  constructor(private api: ApiProcessesService, public dialog: MatDialog) { }
  faPlus = faPlus;
  processes: any = [];

  ngOnInit(): void {
    console.log("init");
    this.getProcessesTimer();
  }

  async getProcessesTimer() {
    while (true) {
      this.getProcesses()
      await this.delay(5000000)
    }
  }

  getProcesses() {
    this.api.getProcesses().subscribe({
      next: (v) => this.processes = (v as ProcessesModel)["processes"],
      error: (e) => this.processes = [],
      complete: () => console.info('complete') 
    });
  }

  addProcess() {
      this.dialog.open(CreateProcessDialogComponent);
  }

  delay(ms: number) {
      return new Promise( resolve => setTimeout(resolve, ms) );
  } 

}
