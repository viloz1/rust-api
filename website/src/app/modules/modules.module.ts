import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LoginModule } from './login/login.module';
import {FormsModule, ReactiveFormsModule} from '@angular/forms';
import { DashboardModule } from './dashboard/dashboard.module';
import { LayoutModule } from '@angular/cdk/layout';
import { SideBarModule } from './side-bar/side-bar.module';


@NgModule({
  declarations: [],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule
  ],
  exports: [LoginModule, DashboardModule, LayoutModule, SideBarModule]
})
export class ModulesModule { }
