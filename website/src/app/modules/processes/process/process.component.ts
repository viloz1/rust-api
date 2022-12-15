import { NgStyle } from '@angular/common';
import { Component, OnInit, Input, EventEmitter, Output, ViewEncapsulation } from '@angular/core';
import { Process } from 'src/app/models/Processes';
import { faCircle } from '@fortawesome/free-solid-svg-icons';
import { ApiProcessesService } from 'src/app/services/api-processes.service';
import { SnackbarService } from 'src/app/design-system/snackbar/snackbar.service';

@Component({
  selector: 'app-process',
  templateUrl: './process.component.html',
  styleUrls: ['./process.component.scss']
})
export class ProcessComponent implements OnInit {

  constructor(private api: ApiProcessesService, private snackbar: SnackbarService) { }
  faCircle = faCircle;
  @Input() process: Process;
  @Output() triggerUpdate: EventEmitter<void> = new EventEmitter<void>();
  color = "";

  ngOnInit(): void {

  }

  ngOnChanges() {
    console.log(this.process);
    switch (this.process.status) {
      case "Off":  
        this.color = 'var(--red)';
        break;
      case 'On':  
        this.color = 'var(--green)';
        break;
      default:
        this.color = 'var(--yellow)';

    }
  }

  start() {
    
    this.api.startProcess(this.process.id).subscribe({
      next: (v) => {
        this.triggerUpdate.emit();
      },
      error: (e) => {
        this.snackbar.openSnackBar("Failed to start process: ","",5000);
        console.log(e)
      },
      complete: () => console.info('complete') 
    });
    
    
  }

  stop() {
    
    this.api.stopProcess(this.process.id).subscribe((result) => {
      console.log(result)
      this.triggerUpdate.emit();
    })
    
    this.api.update(33).subscribe((data) => {
      console.log(data);
    });
  }

  restart() {
    this.api.restartProcess(this.process.id).subscribe((result) => {
      console.log(result)
      this.triggerUpdate.emit();
    })
  }


}
