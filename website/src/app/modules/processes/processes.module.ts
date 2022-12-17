import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ProcessesComponent } from './processes.component';
import { ProcessComponent } from './process/process.component';
import { DesignSystemModule } from 'src/app/design-system/design-system.module';
import { MaterialModule } from 'src/app/design-system/material/material.module';
import { CreateProcessDialogComponent } from './create-process-dialog/create-process-dialog.component';
import { MatDialogModule } from '@angular/material/dialog';



@NgModule({
  declarations: [
    ProcessesComponent,
    ProcessComponent,
    CreateProcessDialogComponent
  ],
  imports: [
    CommonModule,
    MatDialogModule,
    DesignSystemModule
  ]
})
export class ProcessesModule { }
