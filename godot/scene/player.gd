extends Player

func _notification(what) -> void:
	if what == MainLoop.NOTIFICATION_APPLICATION_FOCUS_IN:
		print("On foucs")
	elif what == MainLoop.NOTIFICATION_APPLICATION_FOCUS_OUT:
		print("Lost foucs")
		
