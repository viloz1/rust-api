import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { ApiAuthService } from 'src/app/services/api-auth.service';
import { SidebarRoutes } from './models';

@Component({
  selector: 'app-side-bar',
  templateUrl: './side-bar.component.html',
  styleUrls: ['./side-bar.component.scss']
})
export class SideBarComponent implements OnInit {

  constructor(private auth: ApiAuthService, private router: Router) { }

  routes: SidebarRoutes[] = [
    {
      display: 'Dashboard',
      route: ''
    },
    {
      display: 'Processes',
      route: 'processes'
    }
  ]

  ngOnInit(): void {
  }

  async logout() {
    let r = this.auth.logout();
    r.subscribe({
      next: (v) => this.router.navigate(['/login']),
      error: (e) => console.log("failed to log out"),
      complete: () => console.info('complete') 
    })
  }

}
