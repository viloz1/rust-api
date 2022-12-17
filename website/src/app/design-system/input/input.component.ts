import { Component, ContentChildren, ElementRef, Input, OnInit, QueryList, TemplateRef, ViewChildren } from '@angular/core';
import { FormControl } from '@angular/forms';
import { ErrorStateMatcher } from '@angular/material/core';

@Component({
  selector: 'ds-input',
  templateUrl: './input.component.html',
  styleUrls: ['./input.component.scss']
})
export class InputComponent implements OnInit {

  constructor() { }

  @Input() type: string = "";
  @Input() control: FormControl = new FormControl('');
  @Input() matcher: ErrorStateMatcher = new ErrorStateMatcher();
  @Input() placeholder: string = "";
  @Input() label: string = "";

  ngOnInit(): void {
  }

  @ContentChildren('error') errorChildren: QueryList<TemplateRef<any>> = new QueryList();


}
