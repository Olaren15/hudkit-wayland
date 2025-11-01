#include <gtk/gtk.h>

GtkWidget *window;

// This callback runs when the window is first set to appear on some screen, or
// when it's moved to appear on another.
static void screen_changed(GtkWidget *widget, GdkDisplay *old_display,
                           gpointer userdata) {
  // Die unless the screen supports compositing (alpha blending)
  GdkDisplay *display = gtk_widget_get_display(widget);
  if (!gdk_display_is_composited(display)) {
    fprintf(stderr, "Your screen does not support transparency.\n");
    fprintf(stderr, "Maybe your compositor isn't running?\n");
    exit(2);
  }

  GtkWindow *window = GTK_WINDOW(widget);
}
