/// <reference types="@sveltejs/kit" />

declare namespace App {
  interface Locals {
    session?: boolean;
  }
  interface PageData {
    session?: boolean;
  }
}
