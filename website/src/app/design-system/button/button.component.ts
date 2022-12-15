import { Component, EventEmitter, Input, OnInit, Output, ViewEncapsulation } from '@angular/core';

@Component({
  selector: 'ds-button',
  templateUrl: './button.component.html',
  styleUrls: ['./button.component.scss'],
  encapsulation: ViewEncapsulation.None
})
export class ButtonComponent implements OnInit {

  constructor() { }

  @Output() clickEvent: EventEmitter<void> = new EventEmitter();
  @Input() disabled = false;

  ngOnInit(): void {
  }

}
