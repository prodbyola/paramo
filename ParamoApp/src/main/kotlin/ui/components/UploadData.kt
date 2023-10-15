import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.CornerSize
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

@Composable
fun UploadData(width: Dp){
    var filename by remember { mutableStateOf("") }
    val fieldStyle = Modifier
        .border(
            color = Color.LightGray,
            width = 1.dp,
            shape = CircleShape.copy(CornerSize(12.dp))
        )
        .padding(start = 16.dp, end = 4.dp)

    Column(
        modifier = Modifier.width(width)
    ) {
        Row(
            modifier = fieldStyle,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                "output folder (optional)".lowercase(),
                color = Color.LightGray,
                modifier = Modifier.weight(1f),
            )
            Button(
                onClick = {},
                elevation = null,
                colors = ButtonDefaults.buttonColors(
                    backgroundColor = Color.Transparent
                ),
                shape = CircleShape,
            ){
                Text("Choose Folder", color = Color(0xFF116A7E))
            }
        }
        Spacer(modifier = Modifier.height(8.dp))
        BasicTextField(
            value = filename,
            onValueChange = { filename = it },
            textStyle = TextStyle(fontSize = 16.sp),
            singleLine = true,
            modifier = Modifier
                .height(48.dp),

            decorationBox = { innerTextField ->
                Box(
                    modifier = fieldStyle
                        .fillMaxWidth(),
                    contentAlignment = Alignment.CenterStart
                ) {
                    if (filename.isEmpty()) {
                        Text(
                            text = "output filename (optional)",
                            color = Color.LightGray,
                            fontSize = 16.sp,
//                            modifier = Modifier.padding(8.dp)
                        )
                    }

                    innerTextField()
                }
            }
        )
    }
}