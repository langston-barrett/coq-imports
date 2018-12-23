Require Import UniMath.CategoryTheory.Adjunctions.
Require Import UniMath.CategoryTheory.
Require Import UniMath.Foundations.
Require Import UniMath.CategoryTheory.Equivalences.Core.
Require Import UniMath.CategoryTheory.Whiskering.
Require Import UniMath.Foundations.
Require Import UniMath.

Lemma refl {A : Type} (a : A) : a = a.
Proof.
  reflexivity.
Qed.