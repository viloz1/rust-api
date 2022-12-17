import { Component, OnInit } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import {FormControl, FormGroupDirective, NgForm, Validators} from '@angular/forms';
import {ErrorStateMatcher} from '@angular/material/core';
import { SnackbarComponent } from 'src/app/design-system/snackbar/snackbar.component';
import { SnackbarService } from 'src/app/design-system/snackbar/snackbar.service';
import {map, catchError, throwError} from 'rxjs';
import { ApiAuthService } from 'src/app/services/api-auth.service';
import { Router } from '@angular/router';

export class MyErrorStateMatcher implements ErrorStateMatcher {
  isErrorState(control: FormControl | null, form: FormGroupDirective | NgForm | null): boolean {
    const isSubmitted = form && form.submitted;
    return !!(control && control.invalid && (control.dirty || control.touched || isSubmitted));
  }
}


@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent implements OnInit {

  constructor(
    private http: HttpClient, 
    private snackBarService: SnackbarService,
    private auth: ApiAuthService,
    private router: Router) { }

  ngOnInit(): void {
  }

  emailFormControl = new FormControl('', [Validators.required, Validators.email]);
  passwordFormControl = new FormControl('', Validators.required);

  emailErrors = [
    {
      display: 'Please enter a valid email address', 
      hasError: this.emailFormControl.hasError('email') && !this.emailFormControl.hasError('required')
    },
    {
      display: 'Email is required', 
      hasError: this.emailFormControl.hasError('required')
    }
  ]

  passwordErrors = [
    {
      display: 'Password is required', 
      hasError: this.emailFormControl.hasError('required')
    }
  ]

  matcher = new MyErrorStateMatcher();

  async test() {
    console.log("hej");
    let header: HttpHeaders = new HttpHeaders();
    header = header.set("Content-Type", "application/x-www-form-urlencoded");

    const r = this.http.post("http://localhost:4200/",JSON.stringify({"description":"hej"}), {headers: header});
    r.subscribe((data: any) => {
      console.log(data);
    });
  }

  async check() {
    console.log("hej");
    let header: HttpHeaders = new HttpHeaders();
    header = header.set("Content-Type", "application/x-www-form-urlencoded");

    const r = this.http.post("http://localhost:4200/api/auth/check_login",JSON.stringify({}), {headers: header});
    r.subscribe((data: any) => {
      console.log(data);
    });
  }

  async submit() {
    if(this.emailFormControl.value && this.passwordFormControl.value) {
      let r = this.auth.login(this.emailFormControl.value, this.passwordFormControl.value);
      r.subscribe({
        next: (v) => this.router.navigate(['/']),
        error: (e) => this.snackBarService.openSnackBar("Login failed. Please try again","action",10000),
        complete: () => console.info('complete') 
      })
    }
  }

  async custom() {
    let header: HttpHeaders = new HttpHeaders();

    const r = this.http.get("http://localhost:4200/api/processes/get_processes", {headers: header});
    r.subscribe((data: any) => {
      console.log(data);
    });
  }

}
