import {Injectable} from '@angular/core';
import {Observable, Subject} from 'rxjs';
import { CanActivate, RouterStateSnapshot, ActivatedRouteSnapshot, UrlTree, Router } from '@angular/router';
import { ApiAuthService } from '../services/api-auth.service';

@Injectable()
export class CanActivateAuthenticated implements CanActivate {
    constructor(private router: Router, private auth: ApiAuthService) { }

  canActivate(
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot
  ): Observable<boolean|UrlTree>|Promise<boolean|UrlTree>|boolean|UrlTree {
    let r = this.auth.check_login();
    let answer = new Subject<boolean>();
    r.subscribe((data: any) => {
      console.log(data);
      if(data.user) {
        answer.next(true);
      } else {
        this.router.navigateByUrl("/login")
        answer.next(false);
      }
    });
    return answer;
  }
}