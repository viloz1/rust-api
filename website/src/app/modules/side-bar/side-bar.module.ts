import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SideBarComponent } from './side-bar.component';
import { RouterModule } from '@angular/router';
import { DesignSystemModule } from 'src/app/design-system/design-system.module';



@NgModule({
  declarations: [
    SideBarComponent
  ],
  imports: [
    CommonModule,
    RouterModule,
    DesignSystemModule
  ],
  exports: [SideBarComponent]
})
export class SideBarModule { }
