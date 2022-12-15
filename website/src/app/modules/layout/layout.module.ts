import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LayoutComponent } from './layout.component';
import { BrowserModule } from '@angular/platform-browser';
import { AppRoutingModule } from 'src/app/app-routing.module';
import { RouterModule, Routes } from '@angular/router';
import { DashboardComponent } from '../dashboard/dashboard.component';
import { ModulesModule } from '../modules.module';
import { ProcessesComponent } from '../processes/processes.component';

const routes: Routes = [
  {
    path: '',
    component: DashboardComponent
  },
  {
    path: 'processes',
    component: ProcessesComponent,
    loadChildren: () => import("../processes/processes.module").then(m => m.ProcessesModule),
  }
];

@NgModule({
  declarations: [
    LayoutComponent
  ],
  imports: [
    CommonModule,
    RouterModule.forChild(routes),
    ModulesModule
  ],
  exports:[RouterModule]
})
export class LayoutModule { }
