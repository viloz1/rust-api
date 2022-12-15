import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { LoginComponent } from 'src/app/modules/login/login.component';
import {CanActivateAuthenticated} from 'src/app/auth/authenticated';
import { LayoutComponent } from './modules/layout/layout.component';
import { AppComponent } from './app.component';
import { AlreadyAuthenticated } from './auth/alreadyauthenticated';

const routes: Routes = [];

@NgModule({
  imports: [RouterModule.forRoot([
    {
      path: 'login',
      component: LoginComponent,
      canActivate: [AlreadyAuthenticated]
    },
    {
      path: '',
      component: LayoutComponent,
      loadChildren: () => import("./modules/layout/layout.module").then(m => m.LayoutModule),
      canActivate: [CanActivateAuthenticated]
    }
  ])],
  providers: [CanActivateAuthenticated, AlreadyAuthenticated],
  exports: [RouterModule]
})
export class AppRoutingModule { }
